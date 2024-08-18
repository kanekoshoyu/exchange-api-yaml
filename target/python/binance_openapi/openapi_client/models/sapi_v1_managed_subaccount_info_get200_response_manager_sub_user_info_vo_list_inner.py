# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from typing import Optional, Set
from typing_extensions import Self

class SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner(BaseModel):
    """
    SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner
    """ # noqa: E501
    root_user_id: StrictInt = Field(alias="rootUserId")
    managersub_user_id: StrictInt = Field(alias="managersubUserId")
    bind_parent_user_id: StrictInt = Field(alias="bindParentUserId")
    email: Optional[StrictStr] = None
    insert_time_stamp: StrictInt = Field(alias="insertTimeStamp")
    bind_parent_email: StrictStr = Field(alias="bindParentEmail")
    is_sub_user_enabled: StrictBool = Field(alias="isSubUserEnabled")
    is_user_active: StrictBool = Field(alias="isUserActive")
    is_margin_enabled: StrictBool = Field(alias="isMarginEnabled")
    is_future_enabled: StrictBool = Field(alias="isFutureEnabled")
    is_signed_lvt_risk_agreement: StrictBool = Field(alias="isSignedLVTRiskAgreement")
    __properties: ClassVar[List[str]] = ["rootUserId", "managersubUserId", "bindParentUserId", "email", "insertTimeStamp", "bindParentEmail", "isSubUserEnabled", "isUserActive", "isMarginEnabled", "isFutureEnabled", "isSignedLVTRiskAgreement"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "rootUserId": obj.get("rootUserId"),
            "managersubUserId": obj.get("managersubUserId"),
            "bindParentUserId": obj.get("bindParentUserId"),
            "email": obj.get("email"),
            "insertTimeStamp": obj.get("insertTimeStamp"),
            "bindParentEmail": obj.get("bindParentEmail"),
            "isSubUserEnabled": obj.get("isSubUserEnabled"),
            "isUserActive": obj.get("isUserActive"),
            "isMarginEnabled": obj.get("isMarginEnabled"),
            "isFutureEnabled": obj.get("isFutureEnabled"),
            "isSignedLVTRiskAgreement": obj.get("isSignedLVTRiskAgreement")
        })
        return _obj


